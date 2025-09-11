# SynapseDB

SynapseDB is an intelligent database query system that allows users to interact with databases using simple English sentences instead of writing SQL.  
It uses **DeepSeek NLP models** for natural language processing, **Rust** for high-performance query handling, and **Golang** for scalable backend services.  

---

## Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/DeveloperAromal/SynapseDB.git
cd SynapseDB
````

### 2. Setup Environment

* Copy the example environment file:

```bash
cp .env.example .env
```

* Open `.env` and add your **OpenRouter API Key**.

---

## Run the Project

Start the backend server using Go:

```bash
go run main.go
```

---

## Run Tests

Navigate to the `tests` folder and run:

```bash
cd tests
python test.py
```

---

## Notes

* Ensure you have **Go**, **Python 3**, and **Rust** installed.
* The commit history on [GitHub](https://github.com/DeveloperAromal/SynapseDB) serves as proof of development, showing when the project was started, what changes were made, and by whom.
