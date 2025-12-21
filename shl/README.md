# File Monitor to Clipboard Script

A PowerShell script that monitors a specified file and automatically copies its content to the clipboard when changes occur.

## Features

- Monitors changes to a specified file
- Automatically copies file content to clipboard when it changes and has content
- Displays detailed operation logs with timestamps
- Supports custom check intervals
- Error handling mechanism
- Press Ctrl+C to stop monitoring

## Usage

### Basic Syntax

```powershell
monitor-file-to-clipboard.ps1 -FilePath "D:\path\to\your\file.txt" [-CheckInterval 1000]
```

### Parameter Description

| Parameter Name | Type | Required | Description | Default Value |
|----------------|------|----------|-------------|---------------|
| `-FilePath` | string | Yes | Path to the file to monitor | - |
| `-CheckInterval` | int | No | Interval for checking file changes (milliseconds) | 1000 |

### Usage Examples

#### Example 1: Basic Usage

```powershell
.\monitor-file-to-clipboard.ps1 -FilePath "D:\test\output.txt"
```

- Monitors `D:\test\output.txt` file
- Checks for changes every second by default

#### Example 2: Custom Check Interval

```powershell
.\monitor-file-to-clipboard.ps1 -FilePath "D:\test\output.txt" -CheckInterval 500
```

- Monitors `D:\test\output.txt` file
- Checks for changes every 500 milliseconds (0.5 seconds)

#### Example 3: Using Relative Path

```powershell
.\monitor-file-to-clipboard.ps1 -FilePath ".\logs\app.log" -CheckInterval 2000
```

- Monitors `logs\app.log` file in the current directory
- Checks for changes every 2 seconds

## Output Example

```
Monitoring file: .\test-monitor.txt
Check interval: 500 ms
Press Ctrl+C to stop monitoring...

[2025-12-20 15:00:09] File has new content, copying to clipboard...
Content copied to clipboard
[2025-12-20 15:00:15] File has new content, copying to clipboard...
Content copied to clipboard
```

## Notes

1. **Execution Permission**:
   - Ensure PowerShell allows script execution
   - If you encounter execution permission issues, use this command:
     ```powershell
     powershell.exe -ExecutionPolicy Bypass -File .\monitor-file-to-clipboard.ps1 -FilePath "your-file-path"
     ```

2. **File Path**:
   - Both absolute and relative paths are supported
   - Paths containing spaces need to be enclosed in quotes

3. **Stopping the Script**:
   - Press `Ctrl+C` to stop script execution
   - The script stops immediately and doesn't affect the system clipboard

4. **File Size**:
   - Content is only copied when file size is greater than 0
   - Content is only copied when file size changes (to avoid duplicate copies)

5. **Error Handling**:
   - The script catches and displays error messages
   - Shows a message when the file doesn't exist

## System Requirements

- Windows operating system
- PowerShell 5.1 or later
- .NET Framework 4.5 or later (for `Set-Clipboard` command)

## Application Scenarios

1. **Log Monitoring**: Monitor log files to get the latest log content in real-time
2. **Command Output Monitoring**: Monitor output files from command-line tools to get results in real-time
3. **Text Editor Collaboration**: Monitor shared text files to get others' modifications in real-time
4. **Automated Workflows**: Work with other tools to implement automated data transfer
5. **Debugging Assistance**: Monitor debug output files to easily copy debug information

## Testing the Script with Set-Content

You can use PowerShell's `Set-Content` command to create or modify file content to test the monitoring script's functionality.

### Introduction to Set-Content

The `Set-Content` command is used to write content to a file, overwriting existing file content.

### Basic Syntax

```powershell
Set-Content -Path <File-Path> -Value <Content> [-Encoding <Encoding>]
```

### Using Set-Content with the Monitoring Script

#### Example 1: Create or Overwrite File Content

```powershell
# Create or overwrite file content, triggering the monitoring script to copy to clipboard
Set-Content -Path "D:\test\output.txt" -Value "hello everyone"
```

#### Example 2: Write Multi-line Content

```powershell
# Write multi-line content to the monitored file
Set-Content -Path "D:\test\output.txt" -Value @(
    "First line content",
    "Second line content",
    "Third line content"
)
```

#### Example 3: Write Content Using Variables

```powershell
# Store content in a variable and write to file
$content = "This is content written via a variable"
Set-Content -Path "D:\test\output.txt" -Value $content
```

#### Example 4: Combine with Other Commands

```powershell
# Write the output of another command to the monitored file
Get-Date | Set-Content -Path "D:\test\output.txt"
```

### Notes

- The `Set-Content` command **overwrites** existing file content
- If the file doesn't exist, `Set-Content` creates it automatically
- Supports writing single-line or multi-line content
- Can be used with the pipeline operator to process output from other commands

By using the `Set-Content` command, you can easily test the monitoring script's functionality, verifying whether it can correctly detect file changes and copy content to the clipboard.

## Writing Java Code with Set-Content

You can use PowerShell's `Set-Content` command to create and write Java code files. Here are several commonly used methods:

### 1. Writing Single-line Java Code

For simple single-line Java code, you can write directly using the `-Value` parameter:

```powershell
# Write single-line Java code
Set-Content -Path "Test.java" -Value "public class Test {}"
```

### 2. Writing Multi-line Java Code

For multi-line Java code, you can use an array to store multiple lines:

```powershell
# Write multi-line Java code
Set-Content -Path "Test.java" -Value @(
    "public class Test {",
    "    public static void main(String[] args) {",
    "        System.out.println(\"Hello, World!\");",
    "    }",
    "}"
)
```

### 3. Using Here-String to Write a Complete Java Class

Here-String is a convenient way to write content containing multiple lines, especially suitable for writing complete Java classes:

```powershell
# Use Here-String to write a complete Java class
$javaCode = @"
public class Test {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
"@

Set-Content -Path "Test.java" -Value $javaCode
```

### 4. Writing Java Code with Package Declarations and Import Statements

```powershell
# Write Java code with package declarations and import statements
$javaCode = @"
package com.example;

import java.util.ArrayList;
import java.util.List;

public class Test {
    public static void main(String[] args) {
        List<String> list = new ArrayList<>();
        list.add("Item 1");
        list.add("Item 2");
        
        for (String item : list) {
            System.out.println(item);
        }
    }
}
"@

Set-Content -Path "Test.java" -Value $javaCode
```

### 5. Using Encoding Parameters for Correct Character Encoding

Java files typically use UTF-8 encoding, which you can specify using the `-Encoding` parameter:

```powershell
# Write Java code using UTF-8 encoding
Set-Content -Path "Test.java" -Value $javaCode -Encoding UTF8
```

### 6. Combining with Other Commands to Generate and Write Java Code

```powershell
# Combine with other commands to generate Java code and write to file
$currentDate = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$javaCode = @"
public class Test {
    public static void main(String[] args) {
        System.out.println("Generated on: $currentDate");
    }
}
"@

Set-Content -Path "Test.java" -Value $javaCode
```

### 7. Example: Creating a Complete Java Class File

```powershell
# Create a complete Java class file
$javaClass = @"
/**
 * Test class, used to demonstrate Set-Content command
 * @author Generated by PowerShell
 */
public class TestClass {
    
    // Member variables
    private String name;
    private int age;
    
    // Constructor
    public TestClass(String name, int age) {
        this.name = name;
        this.age = age;
    }
    
    // getter and setter methods
    public String getName() {
        return name;
    }
    
    public void setName(String name) {
        this.name = name;
    }
    
    public int getAge() {
        return age;
    }
    
    public void setAge(int age) {
        this.age = age;
    }
    
    // toString method
    @Override
    public String toString() {
        return "TestClass{" +
                "name='" + name + "'" +
                ", age=" + age +
                '}';
    }
    
    // main method
    public static void main(String[] args) {
        TestClass test = new TestClass("Test", 25);
        System.out.println(test);
    }
}
"@

# Write to file using UTF-8 encoding
Set-Content -Path "TestClass.java" -Value $javaClass -Encoding UTF8
```

### Notes

1. **Filename Matching Class Name**: The Java filename should exactly match the public class name
2. **File Extension**: Java files must use the `.java` extension
3. **Encoding**: It's recommended to use UTF-8 encoding, especially when the code contains non-ASCII characters
4. **Special Characters**: Use escape characters `\` when using quotes in strings
5. **Indentation**: Maintain good indentation to make the code more readable
6. **Package Declarations**: If using package declarations, ensure the file is in the correct directory structure

Using these methods, you can conveniently use PowerShell's `Set-Content` command to create and write Java code files, and combine with the monitoring script to monitor changes to these files in real-time.

## Installation

1. Download the `monitor-file-to-clipboard.ps1` script file
2. Save the script file to any directory
3. Run the script according to the usage instructions above

## License

This script is licensed under the MIT License. You can freely use, modify, and distribute it.

## Author

Created by AI Assistant

## Update Log

### v1.0 (2025-12-20)
- Initial version
- Implemented basic file monitoring functionality
- Supports automatic copying of content to clipboard
- Displays detailed operation logs
- Supports custom check intervals
- Includes error handling mechanism
