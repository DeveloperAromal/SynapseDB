package connection

type ConnStr struct {
	Username string
	Password string
	Host     string
	DBName   string
	SSL      string
}

func ValidateParsedValue(*ConnStr, error) (bool){
	
	



	return  true
}