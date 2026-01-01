RustShell - A new way to catch a shell
---------------------------------------------------------------------------------------------

Disclaimer: This project is designed strictly for educational and ethical purposes and is not
designed for anything other than that. Do not use this tool on systems that you:

    1. Don't have explicit written permission to test
    2. Don't own or aren't deployed by you

The author of this tool assumes no responsibility from misuse.

What is RustShell?
---------------------------------------------------------------------------------------------

This is RustShell - a Project I came up with in my spare time to add to my GitHub portfolio. 

RustShell is a fairly barebones TCP listener designed to catch reverse shells. It technically 
functions no differently from Netcat in terms of catching TCP connections, but includes an 
actual UI and accepts arguments from the CLI to perform its basic functions.

RustShell includes in "in-shell" menu with which you can perform certain functions outside of
the scope of the reverse shell. It also includes a built in syntax generator for a basic reverse
shell. Some supported formats currently among others are:

  - Python3
  - Netcat
  - Nc.exe
  - PHP Pentestmonkey

Basic usage:
----------------------------------------------------------------------------------------------

RustShell can be used with flags from the CLI, or can be started up without flags. To start the
program without flags, type:

  rustshell

If the program is started without flags, it will bring you to the main menu, where you will be 
prompted to enter an option. The currently supported options are:

  1. Start a reverse shell listener
  2. Generate reverse shell syntax
  3. Exit doing nothing

If rustShell is started with flags, they will be treated as though you entered in options from
this menu automatically. The flags currently supported are:

  1. -m or --mode: The mode to use RustShell in. Valid options are either listen, or generate
  2. -f or --format (only applies to generate mode): Specify the format syntax for syntax generator
  3. -s or --shell: Specify the shell you want to use for the format generator
  4. -l or --lport (applies to both listen and generate): The local IP to listen on
  5. -p or --lport (applies to both listen and generate): The local port to listen on

When a connection is established, you will see a prompt indicating you've received a TCP
connection. From there, you can interact with the target system and send commands. RustShell
also includes an "in-shell" menu where you can issue commands to your local system, in case you
need a quick local shell for anything. It also includes other basic functions, such as a help
menu. Other features for the in-shell menu are planned.

To enter the in-shell menu, type "rs:" to bring up the menu prompt. From here, you can type
regular shell commands to execute them on your local system. You can also type "-help" to bring
up a menu of commands. Currently RustShell doesn't include many commands, but I will add more
functionality as I think of them or receive more feedback.

to disconnect the shell at any time, you can enter "exit" into the command menu. This will send
an exit command to the machine and at the same time clean up from the reverse shell. This has 
created a slight problem in that if you have become root on a target machine and type "exit",
The shell will be disconnected and cleaned up. If you're root, you probably shouldn't be reverting
access anyway, but I will fix this in future versions of Rustshell.

Several planned features are coming, among them are:
- The ability to create bind shells instead of only being reverse shells:
  This function will allow you to create a bind shell listener instead of a reverse shell listener
  just to add more options for the end user. Of course, this function will rely upon the target
  system having Rust installed, which can be a case by case issue. I don't see this function being
  used as much as the reverse shell mode, but full functionality is important to me.
- An interactive shell mode inside the "in-shell" menu:
  The current "in-shell" menu is not as interactive as I would like and only includes the ability
  to issue your local system commands. I will currently keep the "in-shell" menu how it is but
  add a new menu option named "-interactive", which will put your session into an actual shell
  session, which will include directory browsing and all that, rather than just the ability to
  execute local commands, in case the user needs a fully interactive local shell. Of course, it is
  pretty easy for the user to just open up another terminal, and in some cases, faster, but "moar
  features", am I right?

Examples:
-----------------------------------------------------------------------------------------------

rustshell - start RustShell with no flags, bringing you directly to the main menu

rustshell -m listen - Start RustShell in listen mode, where you will be prompted for the IP and 
Port to listen on.

rustshell -m listen -l (IP) -p (port) - Start RustShell in listen mode on IP:Port. You will not
have to enter any other menu options, and will immediately start listening on that IP and port.

rustshell -m generate - Start RustShell in generate mode. You will be prompted for the IP, Port,
Format, and shell parameters. After that, RustShell will generate syntax for you to use in your
requested format.

rustshell -m generate -f (Format) -s (Shell) -l (IP) -p (Port) - Start RustShell in generate mode,
You will not be prompted for anything as long as your parameters are valid, and syntax will be 
gerated for you immediately


About the Author:
-----------------------------------------------------------------------------------------------
Hello! If you're using this tool, I appreciate your support! Please keep in mind that I am a 
lone developer. I won't claim that this code is good, but I don't believe this code is bad either.
I have a few more projects in mind that I would like to implement as well, but am currently 
focused on developing RustShell as well as learning other cybersecurity topics. This purpose of 
this project is pure educational. I had decided that I wanted to learn more about how cybersecurity 
tools use TCP connections to perform their functions. When I began this project, I had no idea how 
to build software capable of TCP connections and didn't even know how other software such as Netcat 
performed their basic functions. By building this piece of software, I learned several details:

- Basic programming in the Rust language and how it differs from other languages
- TCP Networking and building software that uses sockets
- How software utilized TCP Data Streams and how to read and write to those streams
- Creating security auditing tools
- CLI Software Design

------------------------------------------------------------------------------------------------
Now, GO AND CATCH THOSE SHELLS!
