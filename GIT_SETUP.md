# Setting Up Local Git and Pushing to GitHub

This guide walks you through setting up Git locally and pushing your project to a GitHub repository.

## Step 1: Create a GitHub Repository

1. Go to https://github.com/new
2. Create a new repository named `word-diff-viewer` (or your preferred name)
3. **Do NOT** initialize it with README, .gitignore, or license (we already have these)
4. Click "Create repository"
5. You'll see a page with commands - copy the HTTPS or SSH URL (the remote URL)

## Step 2: Initialize Git Locally

Open PowerShell in your project directory (`c:\Users\mpin\Projects\rust-1`) and run:

```powershell
# Navigate to the project directory
cd c:\Users\mpin\Projects\rust-1

# Initialize git
git init

# Configure your git user (use your GitHub username and email)
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Add all files to staging
git add .

# Create initial commit
git commit -m "Initial commit: Word Document Diff Viewer"
```

## Step 3: Add Remote and Push

Replace `YOUR_GITHUB_USERNAME` and `YOUR_REPO_NAME` with your actual GitHub username and repository name:

```powershell
# Add the remote repository (use the URL from Step 1)
git remote add origin https://github.com/YOUR_GITHUB_USERNAME/word-diff-viewer.git

# Rename the default branch to main (if needed)
git branch -M main

# Push to GitHub
git push -u origin main
```

## Step 4: Verify the Push

1. Go to your GitHub repository in a web browser
2. You should see all your files uploaded
3. The GitHub Actions workflow should automatically start building

## Checking Build Status

After pushing:
1. Go to your repository on GitHub
2. Click the "Actions" tab at the top
3. You should see the "Build and Release" workflow running
4. Wait for it to complete (usually 5-10 minutes)
5. Once complete, you'll see three artifact files ready for download:
   - `word-diff-viewer-linux`
   - `word-diff-viewer-windows.exe`
   - `word-diff-viewer-macos`

## Downloading Built Executables

### From a Workflow Run
1. Go to the "Actions" tab
2. Click on a completed workflow run
3. Scroll down to "Artifacts"
4. Download the executable for your OS

### From Releases (After Tagged Releases)
When you create a release with a Git tag:
```powershell
# Create a new tag
git tag -a v0.1.0 -m "Version 0.1.0"

# Push the tag to GitHub
git push origin v0.1.0
```
The artifacts will then be attached to a GitHub Release and available for download.

## Common Git Commands

```powershell
# Check status
git status

# See commit history
git log

# Make changes and commit
git add .
git commit -m "Your message"

# Push to GitHub
git push

# Pull latest changes
git pull

# Create a new branch
git checkout -b feature-name

# Switch branches
git checkout main

# Merge a branch
git merge feature-name
```

## Troubleshooting

### "git: command not found"
- Install Git from https://git-scm.com/download/win
- Restart PowerShell after installation

### "fatal: pathspec 'main' did not match any files"
- Use `git branch -a` to see available branches
- The default might be `master` instead of `main`
- Run: `git branch -M main` to rename it

### "Permission denied 'at origin'"
- If using HTTPS: Make sure you entered the correct username/password or use a Personal Access Token
- If using SSH: Ensure your SSH key is added to GitHub (https://docs.github.com/en/authentication/connecting-to-github-with-ssh)

### "fatal: remote origin already exists"
- Remove the old remote: `git remote remove origin`
- Then add the correct one: `git remote add origin <URL>`

## Next Steps

Once you've pushed to GitHub:
1. The builds will run automatically on every push
2. Artifacts are available in the Actions tab
3. You can create releases with version tags
4. Share the GitHub repository URL with others
5. Users can download the precompiled executables without installing Rust!
