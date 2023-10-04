package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"
	"net/url"
	"path/filepath"

	"github.com/gin-gonic/gin"
)

func SetupRouter() *gin.Engine {
	// Disable Console Color
	// gin.DisableConsoleColor()
	r := gin.Default()

	r.POST("/analyze-wf-ss", func(c *gin.Context) {
		screenshotFile, err := c.FormFile("screenshot")
		if err != nil {
			c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{
				"message": "No file is received",
			})
			return
		}

		// validate received file is an image
		buff := make([]byte, 512) // docs tell that it take only first 512 bytes into consideration
		if openedFile, err := screenshotFile.Open(); err != nil {
			if _, err = openedFile.Read(buff); err != nil {
				fmt.Println(err)
				return
			}
		}

		// todo, testing and needs more impl
		fmt.Println("Content Type: " + http.DetectContentType(buff))

		// Retrieve file information
		extension := filepath.Ext(screenshotFile.Filename)

		// Generate random file name for the new uploaded file so it doesn't override the old file with same name
		newFileName := "tempt" + extension

		// Note: this for sure has parallelism issues if this method can run multiple times in a row
		// at least due to file names not being unique it's just tempt right now
		// todo
		if err := c.SaveUploadedFile(screenshotFile, "./"+newFileName); err != nil {
			c.AbortWithStatusJSON(http.StatusInternalServerError, gin.H{
				"message": "Unable to save the file",
			})
			return
		}

		c.JSON(http.StatusOK, gin.H{
			"message": "Your file has been successfully uploaded.",
		})
	})

	return r
}

func ReverseProxy(c *gin.Context) {
	remote, _ := url.Parse("http://localhost:3000")
	proxy := httputil.NewSingleHostReverseProxy(remote)
	proxy.Director = func(req *http.Request) {
		req.Header = c.Request.Header
		req.Host = remote.Host
		req.URL = c.Request.URL
		req.URL.Scheme = remote.Scheme
		req.URL.Host = remote.Host
	}

	proxy.ServeHTTP(c.Writer, c.Request)
}

func main() {
	r := SetupRouter()

	r.NoRoute(ReverseProxy)
	r.Run(":8080")
}
