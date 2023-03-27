CREATE DATABASE Finances;
USE Finances;

Create TABLE Person (
    ID INT NOT NULL PRIMARY KEY,
    Name VARCHAR(255)
);

CREATE TABLE Account (
    ID INT NOT NULL PRIMARY KEY,
    Name VARCHAR(255),
    PersonID INT NOT NULL,
    FOREIGN KEY (PersonID) REFERENCES Person(ID)
);

CREATE TABLE Statement (
    ID INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    TransactionDate DATE NOT NULL,
    PostDate DATE NOT NULL,
    `Description` VARCHAR(2000),
    Category VARCHAR(200),
    `Type` VARCHAR(200),
    Memo VARCHAR(2000),
    Amount DECIMAL(9, 2),
    AccountID INT NOT NULL,
    FOREIGN KEY (AccountID) REFERENCES Account(ID)
);