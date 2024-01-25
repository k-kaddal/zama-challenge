package utils

import (
	"errors"
	"fmt"
	"io"
	"mime/multipart"
	"os"
	"path/filepath"
	"crypto/sha256"
	"encoding/hex"
	"io/ioutil"
)

func Hash(data []byte) string {
	hash := sha256.New()
	hash.Write(data)
	return hex.EncodeToString(hash.Sum(nil))
}

func HashNodes(left, right string) string {
	hash := sha256.New()
	
	leftBytes, err := hex.DecodeString(left)
	if err != nil {
		panic(err)
	}

	rightBytes, err := hex.DecodeString(right)
	if err != nil {
		panic(err)
	}

	hash.Write(leftBytes)
	hash.Write(rightBytes)

	return hex.EncodeToString(hash.Sum(nil))
}

func SaveFiles(files []*multipart.FileHeader) error {
	uploadDir := "./uploads/"

	if err := os.MkdirAll(uploadDir, os.ModePerm); err != nil {
		return err
	}

	for _, file := range files {
		openedFile, err := file.Open()
		if err != nil {
			return fmt.Errorf("Unable to open file: %v", err)
		}
		defer openedFile.Close()

		destFilePath := filepath.Join(uploadDir, file.Filename)

		destFile, err := os.Create(destFilePath)
		if err != nil {
			return fmt.Errorf("Unable to create destination file: %v", err)
		}
		defer destFile.Close()

		_, err = io.Copy(destFile, openedFile)
		if err != nil {
			return fmt.Errorf("Unable to copy file contents: %v", err)
		}
	}

	return nil
}

func GetFile(fileName string) (string, error) {
	uploadsFolder := "./uploads"

	var targetFilePath string

	err := filepath.Walk(uploadsFolder, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		if info.Name() == fileName {
			targetFilePath = path
			return filepath.SkipDir
		}

		return nil
	})

	if err != nil {
		return "", err
	}

	if targetFilePath == "" {
		return "", errors.New("File not found")
	}

	return targetFilePath, nil
}


func GetSingleFileContents(fileName string) ([]byte, error) {
    storagePath := "./uploads" 
    filePath := filepath.Join(storagePath, fileName)

    content, err := ioutil.ReadFile(filePath)
    if err != nil {
        return nil, err
    }

    return content, nil
}

func GetSavedFiles() ([][]byte, error) {
	storagePath := "./uploads"  

	files, err := ioutil.ReadDir(storagePath)
	if err != nil {
		return nil, err
	}

	var fileContents [][]byte

	for _, file := range files {
		filePath := filepath.Join(storagePath, file.Name())
		content, err := ioutil.ReadFile(filePath)
		if err != nil {
			return nil, err
		}
		fileContents = append(fileContents, content)
	}

	return fileContents, nil
}