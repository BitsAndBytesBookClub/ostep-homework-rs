use std::ffi::{c_void, CStr, CString};
use std::{ptr, thread};
use std::fmt::format;
use std::time::Duration;
use libc::{c_int, close, dup2, execv, fdopen, fflush, fork, getpid, O_APPEND, O_RDONLY, O_RDWR, open, pipe, read, STDOUT_FILENO, tcflush, wait, waitpid, WCONTINUED, WNOHANG, write};

pub fn part_1() {
    unsafe {
        println!("hello pid({})", getpid());
        let mut x = 100;
        let rc = fork();
        x = 20;
        if rc < 0 {
            panic!("fork() failed");
        } else if rc == 0 {
            println!("in child: {}", x);
            x = 10;
            println!("in child: {}", x);
            println!("child pid({})", getpid());
        } else {
            println!("in parent: {}", x);
            x = 5;
            println!("in parent: {}", x);
            println!("parent of {}: pid({})", rc, getpid());
        }
    }
}

pub fn part_2() {
    unsafe {
        println!("hello pid({})", getpid());

        let file_name = CString::new("/Users/devan/Documents/BitsAndBytesProjects/ostep/chapter_5/test_file.txt").unwrap();
        let file_ptr = file_name.as_ptr();
        let fd = open(file_ptr, O_RDWR);

        let rc = fork();
        if rc < 0 {
            panic!("failed to fork()");
        } else if rc == 0 {
            let data_to_write= CString::new("HELLO FROM CHILD!\n").unwrap();
            let wd = write(fd, (data_to_write.as_ptr() as *const c_void), 18);
            if wd != 18 {
                panic!("parent did not write to file")
            }
        } else {
            let data_to_write= CString::new("HELLO FROM PARENT!\n").unwrap();
            let wd = write(fd, (data_to_write.as_ptr() as *const c_void), 19);
            if wd != 19 {
                panic!("parent did not write to file")
            }
        }
    }
}

pub fn part_3() {
    unsafe {
        let rc = fork();
        if rc < 0 {
            panic!("there was a problem forking the process");
        }

        if rc == 0 {
            println!("hello");
        } else {
            thread::sleep(Duration::from_millis(100));
            println!("goodbye");
        }
    }
}

pub fn part_4() {
    unsafe {
        let rc = fork();
        if rc < 0 {
            panic!("failed to fork()");
        } else if rc == 0 {
            let exec_name= CString::new("/bin/ls").unwrap();
            let exec_arg = CString::new("-l").unwrap();
            let exec_arg_2 = CString::new("/").unwrap();

            let exec_ptr = exec_name.as_ptr();
            let args = vec![exec_arg.as_ptr(), exec_arg_2.as_ptr(), ptr::null()];

            execv(exec_ptr, args.as_ptr());
        } else {
            wait(&mut WCONTINUED);
        }
    }
}

pub fn part_5_6() {
    unsafe {
        let rc = fork();
        if rc < 0 {
            panic!("there was a problem forking the process");
        }

        if rc == 0 {
            println!("hello");
        } else {
            // Using waitpid() instead of wait() is more flexible.
            // You can set the PID to wait on.

            // wait(&mut WCONTINUED);
            waitpid(rc, &mut WNOHANG, 0);
            println!("goodbye");
        }
    }
}

pub fn part_7() {
    unsafe {
        let rc = fork();
        if rc < 0 {
            panic!("failed to fork()");
        } else if rc == 0 {
            let exec_name= CString::new("/usr/bin/printf").unwrap();
            let exec_arg = CString::new("\"%s\n\"").unwrap();
            let exec_arg_2 = CString::new("\"hi\"").unwrap();

            close(STDOUT_FILENO);
            let exec_ptr = exec_name.as_ptr();
            let args = vec![exec_arg.as_ptr(), exec_arg_2.as_ptr(), ptr::null()];

            execv(exec_ptr, args.as_ptr());
        } else {
            wait(&mut WNOHANG);
        }
    }
}

pub fn part_8() {
    unsafe {
        let mut pipefd: [c_int; 2] = [0; 2];
        if pipe(pipefd.as_mut_ptr()) < 0 {
            panic!("failed to pipe()");
        }

        let rc = fork();
        if rc < 0 {
            panic!("failed to fork()");
        }

        if rc == 0 {
            println!("in child process rc");
            let exec_name = CString::new("/usr/bin/printf").unwrap();
            let exec_arg = CString::new("\"%s\n\"").unwrap();
            let exec_arg_2 = CString::new("\"hi\"").unwrap();

            let exec_ptr = exec_name.as_ptr();
            let args = vec![exec_arg.as_ptr(), exec_arg_2.as_ptr(), ptr::null()];

            close(STDOUT_FILENO);
            dup2(pipefd[1], STDOUT_FILENO);
            close(pipefd[0]);
            close(pipefd[1]);

            execv(exec_ptr, args.as_ptr());
            panic!("execv failed");
        } else {
            let rc_2 = fork();
            if rc_2 < 0 {
                panic!("failed to fork()");
            }

            if rc_2 == 0 {
                println!("in child process rc_2");
                waitpid(rc, ptr::null_mut(), 0);

                close(pipefd[1]);
                let mut buf = [0u8; 1024];
                let n = read(pipefd[0], buf.as_mut_ptr() as *mut _, buf.len());
                if n > 0 {
                    let result = String::from_utf8_lossy(&buf[..n as usize]);
                    println!("Received from rc: {}", result);
                }

                let exec_name = CString::new("/usr/bin/printf").unwrap();
                let exec_arg = CString::new("\"%s\n\"").unwrap();
                let exec_arg_2 = CString::new(format!("\"{}\"", String::from_utf8_lossy(&buf[..n as usize]))).unwrap();

                let exec_ptr = exec_name.as_ptr();
                let args = vec![exec_arg.as_ptr(), exec_arg_2.as_ptr(), ptr::null()];

                execv(exec_ptr, args.as_ptr());
                panic!("execv failed");
            } else {
                println!("in parent process");
                waitpid(rc_2, ptr::null_mut(), 0);
            }
        }
    }
}
