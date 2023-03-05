from user_io import UserInputOutput


def main():
    button1 = 16
    button2 = 20
    button3 = 21
    led1 = 26

    io_manager = UserInputOutput(button1, button2, button3, led1)
    should_run = True

    while should_run:
        buttons = io_manager.poll()
        for i in range(len(buttons)):
            print(f"Button {i} is in state {buttons[i]}")
            print("======")

        if buttons[0]:
            io_manager.led_on()
        else:
            io_manager.led_off()

        if buttons[1]:
            io_manager.toggle_led()

        if buttons[2]:
            should_run = False

    io_manager.cleanup()


if __name__ == '__main__':
    main()
