package utils

import (
			"encoding/binary"
			"fmt"
			"os"
		)

func CreateBin(data uint32) {
	const filePath = "synstore/keys/master.keys.bin"

	if err := os.MkdirAll("synstore/keys", 0755); err != nil {
		panic(fmt.Errorf("failed to create directory: %v", err))
	}

	file, err := os.Create(filePath)
	if err != nil {
		panic(fmt.Errorf("failed to create file: %v", err))
	}
	defer file.Close()

	if err := binary.Write(file, binary.BigEndian, data); err != nil {
		panic(fmt.Errorf("failed to write binary data: %v", err))
	}
}

func ReadBin(path string) uint32 {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(fmt.Errorf("failed to read file: %v", err))
	}

	encoded := binary.BigEndian.Uint32(data)

	return encoded
}

