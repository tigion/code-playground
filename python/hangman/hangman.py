class Hangman:
    max_life_points = 10

    def __init__(self, word):
        self.used_letters = []
        self.search_word = []
        self.life_points = Hangman.max_life_points
        self.wrong_answers = 0
        self.word = word

    @property
    def word(self):
        return self._word

    @word.setter
    def word(self, value):
        if value and not value.isspace():
            self._word = value
            self.init_new_word()
        else:
            raise ValueError("Word cannot be null, empty or contain only white spaces!")

    def init_new_word(self):
        self.search_word = []
        for _ in range(len(self.word)):
            self.search_word.append("_")
        self.used_letters = []
        self.life_points = Hangman.max_life_points
        self.wrong_answers = 0

    def update_search_word(self, letter):
        for i, c in enumerate(self.word):
            if c == letter:
                self.search_word[i] = letter

    def is_letter_already_used(self, letter):
        return True if letter in self.used_letters else False

    def is_letter_in_word(self, letter):
        if self.is_letter_already_used(letter):
            return
        self.used_letters.append(letter)
        self.used_letters.sort()
        if letter in self.word:
            self.update_search_word(letter)
            return True
        else:
            self.wrong_answers += 1
            self.life_points -= 1
            return False

    def is_word_complete(self):
        return True if "_" not in self.search_word else False

    def is_player_dead(self):
        return True if self.life_points <= 0 else False

    def get_search_word_formatted(self):
        return " ".join(self.search_word)

    def get_used_letters_formatted(self):
        return ",".join(self.used_letters)
