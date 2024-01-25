package routes

import (
	"github.com/gorilla/mux"
	"github.com/k-kaddal/zama-server/pkg/controller"
)

func FileRoutes(router *mux.Router) {
	router.HandleFunc("/files/upload", controller.UploadHandler).Methods("POST")
	router.HandleFunc("/files/{file_name}", controller.MerkleProofHandler).Methods("GET")
	router.HandleFunc("/files/{file_name}/download", controller.DownloadHandler).Methods("GET")
}