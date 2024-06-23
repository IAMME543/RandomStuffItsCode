package main

import (
	"fmt"
	"net/http"
	"os"
)

func handler(w http.ResponseWriter, r *http.Request) {
	htmlcontent, err := os.ReadFile("index.html")

	if err != nil {
		http.Error(w, "Failed to read HTML file", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	w.Write(htmlcontent)

}

func main() {
	http.HandleFunc("/", handler)
	fmt.Println("Server Started on :8080")
	http.ListenAndServe(":8080", nil)
}
