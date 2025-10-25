package main

import( 
	"github.com/DeveloperAromal/SynapseDB/api"
	"github.com/DeveloperAromal/SynapseDB/cmd/server"
)

func main() {

	go func() {
		server.Start()
	}()
	
	shell.Runshell()
}
