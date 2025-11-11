package utils

import (
	"encoding/binary"
	"fmt"
	"os"
)

func CreateBin(data string) {
	const filePath = "synstore/keys/master.keys.bin"

	if err := os.MkdirAll("synstore/keys", 0755); err != nil {
		panic(fmt.Errorf("failed to create directory: %v", err))
	}

	file, err := os.Create(filePath)
	if err != nil {
		panic(fmt.Errorf("failed to create file: %v", err))
	}

	defer file.Close()

	length := uint32(len(data))
	if err := binary.Write(file, binary.BigEndian, length); err != nil {
		panic(fmt.Errorf("failed to write length: %v", err))
	}
	
	if _, err := file.Write([]byte(data)); err != nil {
		panic(fmt.Errorf("failed to write data: %v", err))
	}
}

func ReadBin(path string) string {
	file, err := os.Open(path)
	if err != nil {
		panic(fmt.Errorf("failed to open file: %v", err))
	}
	defer file.Close()

	fileInfo, err := file.Stat()
	if err != nil {
		panic(fmt.Errorf("failed to get file info: %v", err))
	}
	if fileInfo.Size() == 0 {
		return ""
	}

	var length uint32
	if err := binary.Read(file, binary.BigEndian, &length); err != nil {
		return ""
	}

	if length > uint32(fileInfo.Size()) || length == 0 {
		return ""
	}

	data := make([]byte, length)
	n, err := file.Read(data)
	if err != nil || n != int(length) {
		return ""
	}

	return string(data)
}
