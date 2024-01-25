package controller

import (
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/k-kaddal/zama-server/pkg/merkle"
	"github.com/k-kaddal/zama-server/pkg/utils"
)

var merkleTree *merkle.MerkleTree

type JSONResponse struct {
	FileName      string   `json:"file_name"`
	MerkleProof   []string `json:"merkle_proof"`
	MerkleRootHash string   `json:"merkle_root_hash"`
}


func UploadHandler(w http.ResponseWriter, r *http.Request) {
	err := r.ParseMultipartForm(10 << 20)
	if err != nil {
		respondWithError(w, "Unable to parse form", http.StatusBadRequest)
		return
	}

	files := r.MultipartForm.File["files"]
	err = utils.SaveFiles(files)

	if err != nil {
		fmt.Printf("Error storing files: %v\n", err)
		respondWithError(w, "Error storing files", http.StatusInternalServerError)
		return
	}

	savedFiles, err := utils.GetSavedFiles()

	if err != nil {
		fmt.Printf("Error getting saved files: %v\n", err)
		respondWithError(w, "Error getting saved files", http.StatusInternalServerError)
		return
	}

	merkleTree = merkle.NewMerkleTree(savedFiles)
	fmt.Printf("Merkle Root Hash: %s\n", merkleTree.Root.Hash)

	respondWithJSON(w, "Documents uploaded successfully", http.StatusOK)
}

func DownloadHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("Downloading\n")
	vars := mux.Vars(r)
	fileName, ok := vars["file_name"]

	if !ok {
		respondWithError(w, "Missing fileName parameter", http.StatusBadRequest)
		return
	}

	targetFilePath, err := utils.GetFile(fileName)

	if err != nil {
		respondWithError(w, fmt.Sprintf("Error: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Disposition", "attachment; filename="+fileName)
	w.Header().Set("Content-Type", "application/octet-stream")

	http.ServeFile(w, r, targetFilePath)
}

func MerkleProofHandler(w http.ResponseWriter, r *http.Request) {
	if merkleTree == nil {
		respondWithError(w, "Merkle tree not initialized", http.StatusInternalServerError)
		return
	}

	vars := mux.Vars(r)
	fileName, ok := vars["file_name"]
	if !ok {
		respondWithError(w, "Missing fileName parameter", http.StatusBadRequest)
		return
	}

	fileContent, err := utils.GetSingleFileContents(fileName)
	if err != nil {
		respondWithError(w, "File not found", http.StatusNotFound)
		return
	}

	targetHash := utils.Hash(fileContent)
	merkleProof, err := merkleTree.GetMerkleProof(targetHash)

	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	fmt.Printf("Merkle Proof for %s: %v\n", fileName, merkleProof)

	merkleRootHash := merkleTree.Root.Hash

	responseData := JSONResponse{
		FileName:      fileName,
		MerkleProof:   merkleProof,
		MerkleRootHash: merkleRootHash,
	}

	respondWithJSON(w, responseData, http.StatusOK)
}
