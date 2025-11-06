package utils


import (
			"encoding/binary"
			"os"
	   )


func createBin(data uint32){
	file, err :=  os.Create("/synstore/keys/master.keys.bin")

	if err != nil {
		panic(err)
	}

	defer file.Close()


	err = binary.Write(file, binary.BigEndian, data)

	if err != nil {
		panic(err)
	}

}