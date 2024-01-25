package main

import (
	"fmt"
	"log"
	"net/http"
	"github.com/gorilla/mux"
	"github.com/k-kaddal/zama-server/pkg/config" 
	"github.com/k-kaddal/zama-server/pkg/routes" 
)

func main() {
	config := config.LoadConfig()

	// Routes
	router := mux.NewRouter()
	apiRouter := router.PathPrefix("/v1/api").Subrouter()
	routes.FileRoutes(apiRouter)

	fmt.Printf("Server is running on port %s...\n", config.Port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%s", config.Port), router))
}
