```
  ____   ___  _        ___                        _              
 / ___| / _ \| |      / _ \ _ __ __ _  __ _ _ __ (_)_______ _ __ 
 \___ \| | | | |     | | | | '__/ _` |/ _` | '_ \| |_  / _ \ '__|
  ___) | |_| | |___  | |_| | | | (_| | (_| | | | | |/ /  __/ |   
 |____/ \__\_\_____|  \___/|_|  \__, |\__,_|_| |_|_/___\___|_|   
                                |___/                            

```
By: Steven Rudenko (steven.gucum@gmail.com)

This is a rust port of this project: https://github.com/TheSuperShell/sql-organizer

This project is inspired by a problem I had at work where I would get billion seperate `.sql` files to execute one by one and had to manually copy them into a one worksheet. This CLI tool automates this process. It also sorts the files based on chosen strategies (I will introduce more in the future). Ofcourse this tool can be used with any file format, but since this is manly used for my SQL problem - I named it SQL Organizer :D

## Examples

Let's say you have a folder `/folder` with a bunch of .sql files (the current directory will be used by default). If you run the following command

```bash
rust-sql-organizer /folder
```

the tool will generate a `target.sql` file with the combined text from all the .sql files in the `/folder` (the search is recursive)

- Specifying file formats

```bash
rust-sql-organizer /folder -e txt -e sql
```

This will find all the files with .txt and .sql formats in the `/folder` folder

- Specifying target path

```bash
rust-sql-organizer /folder -t /my_target.sql
```

this will put the result into the `my_target.sql` file. The command will error if the file already exists by default, if you want to overwrite it put `--overwrite` parameter:

```bash
rust-sql-organizer /folder -t /my_target.sql --overwrite
```

- Choose sorting strategies

```bash
rust-sql-organizer /folder -s last_number
```

You can also chain the startegies, the priority is determined by the position, higher to lower priority
Here are all the available strategies for now

| **Stategy**  | **Descrition**                                                                                                                                                         |
|--------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| first_number | The ordering is based on the fist number found in the file name. 0 will be used if no number is found<br>Example: `100_SCRIPT_2.sql` -> 100 will be used for sorting   |
| last_number  | The ordering is based on the last number found in the file name. 0 will be used if no number is found<br>Example: `100_SCRIPT_2.sql` -> 2 will be used for sorting     |
| folder       | The name of the last folder will be used for soring. Sorting is done in an alphabetical order<br>Example: `my/folder/100_SCRIPT.sql` -> folder will be used for soring |

- Write output to the file

Use parameter `--out` with the file path to write the utput into the file

```bash
rust-sql-organizer /folder --out output.log
```


