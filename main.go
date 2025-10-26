package main

import (
	api "github.com/DeveloperAromal/SynapseDB/api"
	shell "github.com/DeveloperAromal/SynapseDB/cmd/server"
)

func main() {

	ready := make(chan struct{})
	go api.Start(ready)
	<-ready

	shell.Runshell()
}
