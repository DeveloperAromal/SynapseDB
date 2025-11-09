package utils

import (
			"fmt"
	   )

func NShell_banner(){
	fmt.Printf("\033[38;5;208m")
	fmt.Println("======================================================")
	fmt.Println("-------------------- Synapse Shell -------------------")
	fmt.Println("======================================================")
	fmt.Printf("\033[0m\n")
}


func NShell_NewUserBanner(){
	fmt.Printf("\033[38;5;208m")
	fmt.Println("╔════════════════════════════════════════════════════════════╗")
	fmt.Println("║                Welcome to Synapse Shell                    ║")
	fmt.Println("╠════════════════════════════════════════════════════════════╣")
	fmt.Printf("\033[0m")
	fmt.Printf("\033[38;5;208m")
	fmt.Println("║ It looks like this is your first time using SynapseDB.     ║")
	fmt.Println("║                                                            ║")
	fmt.Println("║ Here are a few quick tips to get started:                  ║")
	fmt.Println("║  • Use 'help' to see available commands.                   ║")
	fmt.Println("║  • Create a table using 'create table <name>'.             ║")
	fmt.Println("║  • Insert data with 'insert into <table>'.                 ║")
	fmt.Println("║  • Type 'exit' to close the shell.                         ║")
	fmt.Println("║                                                            ║")
	fmt.Println("║ Explore, experiment, and enjoy working with your data!     ║")
	fmt.Printf("\033[38;5;208m")
	fmt.Println("╚════════════════════════════════════════════════════════════╝")
	fmt.Printf("\033[0m")
}

