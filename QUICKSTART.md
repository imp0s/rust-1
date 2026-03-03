# Quick Start - Git and GitHub

## TL;DR - Get Started in 5 Minutes

### 1. Create GitHub Repo
- Go to https://github.com/new
- Name it `word-diff-viewer`
- Click "Create" (leave all boxes unchecked)
- Copy the HTTPS URL shown

### 2. Initialize Git Locally
```powershell
cd c:\Users\mpin\Projects\rust-1
git init
git config user.name "Your Name"
git config user.email "your@email.com"
git add .
git commit -m "Initial commit"
```

### 3. Push to GitHub
```powershell
# Paste YOUR repo URL here (from GitHub)
git remote add origin https://github.com/YOUR_USERNAME/word-diff-viewer.git

git branch -M main
git push -u origin main
```

### 4. Download Built Executables
- Wait 5-10 minutes for build to complete
- Go to GitHub repo → Actions tab
- Click the green checkmark workflow
- Download the .exe (Windows), Linux, or macOS version

---

## Prerequisites

**Install Git first:**
- Download from https://git-scm.com/download/win
- Run installer, accept defaults
- Restart PowerShell

---

## What Happens Next

✅ GitHub Actions automatically builds your code  
✅ Creates executables for Windows, Linux, and macOS  
✅ Makes them available for download  
✅ No need to install Rust locally!

---

See [GIT_SETUP.md](GIT_SETUP.md) for detailed instructions.
