import { ISocLink } from "shared/ui/SocLink";
import { INavLink } from "../../types";

export const socLinks: ISocLink[] = [
  {
    alt: "Telegram",
    href: "https://t.me/ne0chad",
    img: "/icons/tg.svg",
  },
  {
    alt: "Discord",
    href: "https://discordapp.com/users/543459187460669471",
    img: "/icons/discord.svg",
  },
];

export const navLinks: INavLink[] = [
  {
    img: "/icons/pyramid.svg",
    to: "/",
    subtitle: "Main",
  },
  {
    img: "/icons/keyboard.svg",
    to: "/typing",
    subtitle: "Typing",
  },
];
