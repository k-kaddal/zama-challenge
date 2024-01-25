package controller

import (
	"encoding/json"
	"net/http"
)

func respondWithError(w http.ResponseWriter, message string, statusCode int) {
	http.Error(w, message, statusCode)
}

func respondWithJSON(w http.ResponseWriter, data interface{}, statusCode int) {
	jsonResponse, err := json.Marshal(data)
	if err != nil {
		respondWithError(w, "Error creating JSON response", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(statusCode)
	_, err = w.Write(jsonResponse)
	if err != nil {
		respondWithError(w, "Error writing JSON response", http.StatusInternalServerError)
		return
	}
}

// func handleFileError(w http.ResponseWriter, err error, errorMessage string, statusCode int) {
// 	fmt.Printf("Error: %v\n", err)
// 	respondWithError(w, errorMessage, statusCode)
// }

