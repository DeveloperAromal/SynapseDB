//go:build !cgo
// +build !cgo

package execute

// Execute is a fallback stub used when cgo is disabled or unavailable.
func Execute(query string) string {
	return "Error: cgo is disabled or unavailable. Enable CGO or install a C toolchain to use the Rust engine."
}
