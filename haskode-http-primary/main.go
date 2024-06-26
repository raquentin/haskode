package main

import (
    "fmt"
    "log"
    "net/http"
    "os"

    "github.com/gorilla/mux"
    "golang.org/x/oauth2"
    "golang.org/x/oauth2/github"
)

var (
    githubOAuthConfig = &oauth2.Config{
        ClientID:     os.Getenv("GITHUB_CLIENT_ID"),
        ClientSecret: os.Getenv("GITHUB_CLIENT_SECRET"),
        RedirectURL:  "http://localhost:8080/callback",
        Scopes:       []string{"user:email"},
        Endpoint:     github.Endpoint,
    }
    oauthStateString = "random"
)

func main() {
    r := mux.NewRouter()
    r.HandleFunc("/login", handleGitHubLogin)
    r.HandleFunc("/callback", handleGitHubCallback)

    // Other routes for problem listing, submission, etc.

    log.Fatal(http.ListenAndServe(":8080", r))
}

func handleGitHubLogin(w http.ResponseWriter, r *http.Request) {
    url := githubOAuthConfig.AuthCodeURL(oauthStateString)
    http.Redirect(w, r, url, http.StatusTemporaryRedirect)
}

func handleGitHubCallback(w http.ResponseWriter, r *http.Request) {
    code := r.URL.Query().Get("code")
    if code == "" {
        http.Error(w, "Code not found", http.StatusBadRequest)
        return
    }

    token, err := githubOAuthConfig.Exchange(oauth2.NoContext, code)
    if err != nil {
        http.Error(w, "Failed to exchange token: "+err.Error(), http.StatusInternalServerError)
        return
    }

    // You can now use the token to get the user's information.
    fmt.Fprintf(w, "Login successful with token: %s", token.AccessToken)
}
