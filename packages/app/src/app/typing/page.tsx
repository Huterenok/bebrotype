import { Metadata } from "next";

import { Title } from "shared/ui";

import styles from "app-flat/styles/page-styles/Typing.module.css";
import { ITypingCard } from "widgets/TypingCard/types";
import { TypingCard } from "widgets/TypingCard/ui";

export const metadata: Metadata = {
  title: "Typing",
};

//TODO: Take away this to another place
const typingStyles: ITypingCard[] = [
  {
    icon: "/icons/freedom.svg",
    title: "Free Typing",
    subtitle: "Text from a variety of words for the most free people",
    href: "/typing/freedom",
  },
  {
    icon: "/icons/group.svg",
    title: "User's texts",
    subtitle: "Ready-made texts from people of different stripes",
    href: "/typing/texts",
  },
];

export default function TypingPage() {
  return (
    <div className={styles.wrapper}>
      <Title>Choose your typing style</Title>
      <div className={styles.styles}>
        {typingStyles.map((el) => (
          <TypingCard {...el} key={el.href} />
        ))}
      </div>
    </div>
  );
}