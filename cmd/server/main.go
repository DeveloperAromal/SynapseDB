package shell


import (
		"fmt"
		"net"
		"time"
		"log"
		
	   )

func Runshell(){

	host := "127.0.0.1"
	port := "4538"


	fmt.Printf(`
	======================================================
					    Synapse Shell 
	======================================================
	         `)		

	addr := net.JoinHostPort(host, port)

	start := time.Now()

	conn, err := net.Dial("tcp", addr)

	if err != nil {
		log.Fatal("Connection error:", err)
		return
	}

	defer conn.Close()

	fmt.Printf("Connected to %s\n", addr)


    var query string
	
	fmt.Print("$> ")
	fmt.Scanln(&query)


	_, err = conn.Write([]byte(query))

	if err != nil {
		log.Fatal("send error:", err)
		return
	}


	buffer := make([]byte, 1024)

	n, err := conn.Read(buffer)

	if err != nil {
		log.Fatal("read error:", err)
		return
	}

	
	end := time.Now()

	latency := end.Sub(start).Milliseconds()


	fmt.Printf("Latency: ~%dms\n", latency)
	fmt.Printf("Received: %s\n", string(buffer[:n]))

}
