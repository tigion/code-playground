import random

import wordlist
from hangman import Hangman


# Returns a random word from the passed word list.
def get_random_word(words):
    return words[random.randint(0, len(words) - 1)].upper()


# Returns true if the passed letter is valid, otherwise false.
def is_letter_valid(letter):
    valid_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    if len(letter) != 1 or not letter.isalpha():
        print("> Bitte nur einzelne Buchstaben eingeben!")
        return False
    elif letter not in valid_letters:
        print("> Bitte keine Umlaute wie 'ÄÖÜ' oder ein 'ß' eingeben!")
        return False
    return True


# main
def main():
    # Init a Hangman instance with a random word.
    hm = Hangman(get_random_word(wordlist.words))

    print("Willkommen zu Galgenraten!")
    print("\nGesucht wird ein Wort mit " + str(len(hm.word)) + " Buchstaben.")

    # Endless loop
    while True:
        # Print and draw infos
        print("\n" + hm.get_search_word_formatted() + "\n")
        print(str(hm.life_points) + "/" + str(hm.max_life_points) + " Leben")
        print("Verwendet: " + hm.get_used_letters_formatted())

        # Check end condition for endless loop
        if hm.is_word_complete():
            print("\nGewonnen!\nDu hast '" + hm.word + "' richtig erraten.")
            break
        elif hm.is_player_dead():
            print("\nVerloren!\nDu hast '" + hm.word + "' leider nicht erraten.")
            break

        # Input a letter and check if it is valid
        letter = input("Welcher Buchstabe ist enthalten? ").upper()
        if not is_letter_valid(letter):
            continue

        # Check if letter is already used
        if hm.is_letter_already_used(letter):
            print("> Aufpassen, schon verwendet!")
            continue

        # Check if letter is in word
        if hm.is_letter_in_word(letter):
            print("> Richtig :)")
        else:
            print("> Falsch, du verlierst ein Leben :(")


if __name__ == "__main__":
    main()
