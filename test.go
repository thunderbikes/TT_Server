package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
)

const port = 8080

type Entry struct {
	Description string
	Urgency     int32
	Area        string
}

type dictionary struct {
	entries map[int32]Entry
}

func (d *dictionary) addWord(code int32, definition Entry) {
	d.entries[code] = definition
}

func (d *dictionary) remove(code int32) {
	delete(d.entries, code)
}

func (d *dictionary) GetEntry(code int32) (Entry, bool) {
	e, ok := d.entries[code]
	return e, ok
}

type ErrorData struct {
	Description string
	Time        int64
	Urgency     int32
	Area        string
}

type data struct {
	errors map[int32]ErrorData
}

func (d *data) addError(number int32, e ErrorData) {
	d.errors[number] = e
}

func (d *data) remove(number int32) int32 {
	if _, ok := d.errors[number]; !ok {
		return -1
	}
	delete(d.errors, number)
	return number
}

func (d *data) AddError(number int32, e ErrorData) {
	d.errors[number] = e
}

type Dictionary struct {
	entries map[string]string
}

func (d *dictionary) addWord(word, definition string) {
	d.entries[word] = definition
}

func (d *dictionary) Remove(word string) {
	delete(d.entries, word)
}

func (d *dictionary) GetDefinition(word string) (string, bool) {
	definition, ok := d.entries[word]
	return definition, ok
}

func main() {
	errorDictionary := Dictionary{entries: make(map[int32]Entry)}
	errorDictionary.addWord(1, Entry{Description: "overcurrent", Urgency: 0, Area: "BMS"})
	errorDictionary.addWord(2, Entry{Description: "overtemp", Urgency: 0, Area: "Motor"})
	errorDictionary.addWord(3, Entry{Description: "overvoltage", Urgency: 0, Area: "BMS"})

	data := Data{errors: make(map[int32]ErrorData)}

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		command := ""
		inputCode := 0
		fmt.Sscanf(path, "/%s/%d", &command, &inputCode)

		switch command {
		case "version":
			fmt.Fprintln(w, "0.1.0")
		case "get":
			w.Header().Set("Content-Type", "application/json")
			w.Write(data.makeJSON())
		case "add":
			entry := errorDictionary.GetEntry(inputCode)
			if entry.Description == "" {
				fmt.Fprintln(w, "Invalid")
			} else {
				data.addError(inputCode, Error{Description: entry.Description, Time: time.Now().UTC().Unix(), Urgency: entry.Urgency, Area: entry.Area})
				fmt.Fprintln(w, inputCode)
			}
		default:
			fmt.Fprintln(w, "Invalid command")
		}
	})

	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatal(err)
	}
}
