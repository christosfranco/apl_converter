def main():
    matrix = [
        [1,2, 3],
        [4,5,6 ]
    ]

    for row in matrix:
        # newline after each row
        print(' '.join(map(str, row)))
        
if __name__ == "__main__":
    main()
    print('')
