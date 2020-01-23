import random
import string


NUMBER_OF_CITIES = 15
HOTELS_PER_CITY = 100000
ROOMS_PER_HOTEL = 10
RESERVATION_PER_ROOM = 20


class CustomRandom(object):
    characters: str = string.ascii_lowercase
    range: range = range(0, 5)

    def __init__(self, min_length: int = 0, max_length: int = 5, characters: str = string.ascii_lowercase):
        super().__init__()

        self.characters = characters
        self.range = range(random.choice(range(min_length, max_length+1)))

    def gen(self) -> str:
        return ''.join(random.choice(self.characters) for _ in self.range)


def main():
    city_names_gen = CustomRandom(3, 3, string.ascii_uppercase)
    hotel_rooms_gen = CustomRandom(4, 5, string.ascii_uppercase)

    city_names = list(dict.fromkeys([city_names_gen.gen() for _ in range(NUMBER_OF_CITIES)]))
    hotel_rooms = list(dict.fromkeys([hotel_rooms_gen.gen() for _ in range(ROOMS_PER_HOTEL)]))

    hotels = open('hotels_2.json', 'w')
    room_names = open('room_names_2.csv', 'w')
    input = open('input_2.csv', 'w')

    input.write('city_code|hotel_code|room_type|room_code|meal|checkin|adults|children|price|source\n')

    range_hotels_per_city = range(HOTELS_PER_CITY)
    range_hotel_rooms = range(len(hotel_rooms))
    range_reservation_per_room = range(RESERVATION_PER_ROOM)

    for city_name in city_names:
        for hotels_index in range_hotels_per_city:
            hotels.write('{"id": "'+city_name+f'{hotels_index:05}'+'", "city_code": "'+city_name+'", "name": "Crowne Plaza Berlin City Centre", "category": 4.0, "country_code": "DE", "city": "Berlin" }\n')
            for room_names_index in range_hotel_rooms:
                room_names.write(city_name+f'{hotels_index:05}'+'|'+hotel_rooms[room_names_index]+'|Deluxe King Extra|'+city_name+f'{room_names_index:03}'+'\n')
                for _ in range_reservation_per_room:
                    input.write(city_name+'|'+city_name+f'{hotels_index:05}'+'|DZ|'+city_name+f'{room_names_index:03}'+'|U|20180723|2|1|176.01|'+hotel_rooms[room_names_index]+'\n')

    hotels.close()
    room_names.close()
    input.close()


if __name__ == "__main__":
    main()
