from random import randrange

rps_map = [
    "rock",
    "paper",
    "scissors"
]

gamestate_map = [
    "you won!",
    "you lost :(",
    "it was a tie"
]

def looping(func):
    s = object()
    
    def inner():
        t = None

        while s == (t := func(s)):
            pass
        
        return t
    
    return inner
    

def rps_compare(a, b):
    if a == 0 and b == 2:
        return 0
    if a == 2 and b == 0:
        return 1
    if a == b:
        return 2
    if a > b:
        return 0
    if a < b:
        return 1

def get_rand_rps():
    return randrange(0, 3)

@looping
def get_player_rps(s):
    player = input("Enter your choice: ")

    if player == rps_map[0]:
        return 0
    if player == rps_map[1]:
        return 1
    if player == rps_map[2]:
        return 2
    
    print("Not a valid choice. Try again.")

    return s

@looping
def main_loop(s):
    player = get_player_rps()
    computer = get_rand_rps()

    print(f"Your choice:     {rps_map[player]}")
    print(f"Computer choice: {rps_map[computer]}")

    res = rps_compare(player, computer)

    print(f"Result:          {gamestate_map[res]}")

    print()

    print("Type 'yes' to play again.")

    return s if input() == "yes" else None

def main():
    print("Welcome to nekodjin's Rock, Paper, Scissors!")
    print("Press Enter to begin.")

    input()

    main_loop()

    print("Bye bye :)")

if __name__ == "__main__":
    main()

