# subtree

## tomlyre-img

```sh
git remote add tomlyre 2moe:2moe/tomlyre.git

git subtree add -P assets/tomlyre-img tomlyre img --squash
# git mv tomlyre-img assets
```

## theme-syntax-set

```sh
git remote set-url --add origin 2moe:2moe/hlight.git

git subtree add -P hlight/assets/theme-syntax-set origin theme-syntax-set --squash
```

## elvish

```sh
git remote add elvish_syntax_for_sublime https://github.com/href/elvish_syntax_for_sublime.git

git subtree add -P assets/syntax/elvish/git elvish_syntax_for_sublime master --squash
```

<!--
## powershell

```sh
git remote add pwsh_syntax https://github.com/PowerShell/EditorSyntax

git subtree add -P assets/syntax/powershell/git pwsh_syntax master --squash
```

## sublime

```sh
git remote add sublimehq https://github.com/sublimehq/Packages
git subtree add -P assets/syntax/sublime/git sublimehq 759d6eed9b4beed87e602a23303a121c3a6c2fb3 --squash
```
-->
