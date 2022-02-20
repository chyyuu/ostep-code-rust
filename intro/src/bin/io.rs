use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut f = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open("./src/bin/tmpfile")
        .unwrap();
    f.write_all(b"Hello, world\n").unwrap();
    f.sync_all().unwrap();
}

/*
#include <stdio.h>
#include <unistd.h>
#include <assert.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <string.h>

int main(int argc, char *argv[]) {
    int fd = open("/tmp/file", O_WRONLY | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    assert(fd >= 0);
    char buffer[20];
    sprintf(buffer, "hello world\n");
    int rc = write(fd, buffer, strlen(buffer));
    assert(rc == (strlen(buffer)));
    fsync(fd);
    close(fd);
    return 0;
}

 */
