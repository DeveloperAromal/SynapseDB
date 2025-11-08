package security


import (
			"bufio"
			"os"
			"fmt"
			"strings"
	   )


func create_acc() (string, string){
	reader := bufio.NewReader(os.Stdin)


	fmt.Print("Enter a new username: ")
	username, err := reader.ReadString('\n')

	if err != nil {
		panic(err)
	}

	fmt.Print("Enter a new password: ")
	password, err := reader.ReadString('\n')


	
	if err != nil {
		panic(err)
	}

	fmt.Print("Confirm password: ")
	c_password, err := reader.ReadString('\n')


	
	if err != nil {
		panic(err)
	}



	username = strings.TrimSpace(username)
	password = strings.TrimSpace(password)
	c_password = strings.TrimSpace(c_password)


	if password == c_password {
		fmt.Print("")
	} else{
		fmt.Print("Password does not match the previous entered")
	}



	return username, password
}

