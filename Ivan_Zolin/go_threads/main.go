package main

import (
	"bufio"
	"fmt"
	"net/http"
	"os"
	"sync"
)

func fetchData(url string, wg *sync.WaitGroup, results chan string) {
	defer wg.Done()

	resp, err := http.Get(url)
	if err != nil {
		fmt.Printf("Error fetching data from %s: %v\n", url, err)
		results <- fmt.Sprintf("Error fetching data from %s: %v", url, err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		fmt.Printf("HTTP error from %s: %s\n", url, resp.Status)
		results <- fmt.Sprintf("HTTP error from %s: %s", url, resp.Status)
		return
	}

	results <- fmt.Sprintf("Fetched data from %s", url)
}

func main() {
	// Open the file
	file, err := os.Open("urls.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	var urls []string

	// Read the URLs from the file
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		urls = append(urls, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var wg sync.WaitGroup
	results := make(chan string, len(urls))

	for _, url := range urls {
		wg.Add(1)
		go fetchData(url, &wg, results)
	}

	wg.Wait()
	close(results)

	for result := range results {
		fmt.Println(result)
	}
}
