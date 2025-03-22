# Tap - CLI for Quick Link Access

Tap is a command-line tool designed to help you quickly access links and resources associated with a parent entity (repositories, projects, etc.). Ditch the endless scrolling in browser bookmarks and let tap find the right link instead! 

Wait you don't use the Internet and still prefer to store everything locally? That's cool too, Tap also support file navigation. Pick up right where you left off with the ease of one command, for example: `tap documents expense-report`.

_DISCLAIMER: Tap is a work in progress and is under active development. Not all features listed below are available at this time. See the full [roadmap here](./ROADMAP.md) for what is implemented so far. If you do test this out and find bugs/issues, please add an issue to the repository. Thank you!_

## Key Features

- **Context-Aware Commands**
  - If you’re already inside a stored repository directory, you don’t need to specify the repo name. Instead, tap provides the `here` keyword to autofill this context, for example `tap here secret` will retrieve the associated secret link stored under the current repository, if it exists.
  - *TODO* add in file tree and table showing what is stored to make the provided example more clear
- **Flexible Use Cases**
  - Originally designed for Software Engineers with way too many repositories, Tap specializes in quickly accessing build artifacts, logs, and pipeline dashboards without searching for links. However, Tap is not limited to this one usecase. Tap's design adapts well to both personal and enterprise use cases where directories & files on a local computer have couterparts online.
  - Examples:
    - Software Engineers
      - Did my pushed code just succeed or fail? Let's find out with `tap here build`
      - Demo looks great! Time to merge and deploy. Simplify the link hunting with: `tap here repo` and `tap here deploy`
      - Hey, can you send me the API documentations for this repo? I can't find them in the README. Say no more: `tap here api-docs`
      - 401 Unauthorized :(, time to get new secrets: `tap my-repo secrets`
    - Enterprise
      - Have a bunch of stored spreadsheets on your computer, but your co-workers store new files in an enterprise cloud solution like OneDrive? You can use Tap to quickly navigate to the cloud solution using a command like `tap my-spreadsheets onedrive`
    - Students & Academics
      - Are you that student that always loses the link to that one paper that was perfect for your research paper? Let Tap remember for you, so you can focus on getting that A+ instead: `tap documents resource-1`
      - Did you forget about that quiz due at 11:59 pm? Me too! Let Tap quickly get you to the online portal to finish your quiz: `tap university canvas`
- **CRUD Operation Support**
  - Easily add, update, remove, or list your parent entities and associated links via our CLI command `tap -l` or `tap --list`
  - Prefer a UI... Tap has an interactive terminal user interface as well `tap --tui` 
- **Auto-Completion**
  - What CLI doesn't have auto complete these days? With Tap, the goal is to get you where you want to go fast. Tap dynamically generates auto-complete suggestions whenever you hit tab, its that simple!
  - Say you are typing `tap my-`
    - Tap will look through your parent entities and see if there's any matches. If there,s multiple, Tap lists them out to help you narrow down your search. Otherwise, it auto-fills to move you along in your command
  - Of course Tap also supports this for links within your parent entities. For example, say you type `tap my-repository secre` and `my-repository` only has one link named `secrets`. Hitting tab, Tap will auto complete the typing of `secrets` for you.
- **Easy Onboarding Via Bulk Import**
  - Bookmark managers have been around for years. Knowing this, Tap allows imports of the following browsers' bookmark manager files
    - Chrome, Edge, Firefox, Opera, Safari
  - So your a programmer that wants to generate your own file of links into Tap? That's awesome, and also supported using YAML syntax. Below makes one new parent-entity called `tmgr` with a `repository` link (feel free to check out `tmgr` if your looking to manage tasks using a CLI!)
    ```yaml
    - tmgr:
        repository: https://github.com/CharlieKarafotias/tmgr/tree/main
    ``` 
- **Easily Migrate To A Browser Bookmark Manager**
  - So you have moved on from the terminal. That's okay, Tap can compile all your links down to a file for a quick exit.
  - Tap supports migrating to the following browsers' bookmark managers:
    - Chrome, Edge, Firefox, Opera, Safari 
- **Multi-OS Support**
  - Tap is offered on MacOS and UNIX platforms at this time.
  - In the future, Windows will also be supported. 
- **Built-in Updater**
  - You will always stay up to date with the built in `tap update` command.
 
## Reserved Keyword

Given the features provided by `tap` out of the box, some key words must be reserved. 

Reserved List:
- `here`
- `update`

### How Does This Affect Me?

It most likely won't! The only way reserved words will affect you is if you attempt to create a new parent entity using one of the reserved key words. In the case you accidently do, an error will be raised, allowing you to edit your name.

