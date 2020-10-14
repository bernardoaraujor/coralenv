#include <stdio.h>

#include "coralenv.h"

int main(int argc, char *argv[]) {
    printf("Temperature: %f °C\n", temperature());
    printf("Humidity: %f %\n", humidity());
    printf("Ambient Light: %f lux\n", light());
    printf("Pressure: %f kPa\n", pressure());
}
