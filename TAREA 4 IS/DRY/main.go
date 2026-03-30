package main

import (
	"fmt"
	"strings"
)

func emailValido(email string) bool {
	return email != "" && strings.Contains(email, "@")
}

func registrarUsuario(email string) bool {
	if !emailValido(email) {
		return false
	}
	return true
}

func actualizarPerfil(email string) bool {
	if !emailValido(email) {
		return false
	}
	return true
}

func main() {
	fmt.Println("registrar:", registrarUsuario("ana@mail.com"))
	fmt.Println("perfil:", actualizarPerfil("sin-arroba"))
}
