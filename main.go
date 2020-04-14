package main

import (
    "fmt"
    "log"
    "os"
    "io/ioutil"
    "net/http"
    "time"

    "github.com/gorilla/mux"
)


func home (w http.ResponseWriter, r *http.Request) {
    http.ServeFile(w, r, "www/index.html")
}

func listWords (w http.ResponseWriter, r *http.Request) {
    http.ServeFile(w, r, "wordlist")
}

func addWords (w http.ResponseWriter, r *http.Request) {
    b, err := ioutil.ReadAll(r.Body)
    defer r.Body.Close()
    if err != nil {
        http.Error(w, err.Error(), 500)
        return
    }

    f, err := os.OpenFile("wordlist", os.O_APPEND|os.O_WRONLY, 0600)
    defer f.Close()
    if err != nil {
        log.Fatal(err.Error())
    }

    str := string(b) + ","
    fmt.Println(str)
    if _, err = f.WriteString(string(b) + ","); err != nil {
        log.Fatal(err.Error())
    }
}

func reset (w http.ResponseWriter, r *http.Request) {
    os.Rename("wordlist", "lists/" + time.Now().Format("20060102150405"))
    f, err := os.Create("wordlist")
    defer f.Close()
    if err != nil {
        log.Fatal(err.Error())
    }
}

func handleRequests() {
    router := mux.NewRouter().StrictSlash(true)
    router.HandleFunc("/", home)
    router.HandleFunc("/words", addWords).Methods("POST")
    router.HandleFunc("/words", listWords)
    router.HandleFunc("/reset", reset)
    
    log.Fatal(http.ListenAndServe(":3000", router))
}

func main() {
    handleRequests()
}
