import { FC } from "react";

import { typingStyles } from "../config";

import { Title, TypingCard } from "shared/ui";
import styles from "./ChoosingStyle.module.scss";

export const ChoosingStyle: FC = () => {
  return (
    <>
      <Title>Choose your typing style</Title>
      <div className={styles.styles}>
        {typingStyles.map((el) => (
          <TypingCard
            href={el.href}
            icon={el.icon}
            subtitle={el.subtitle}
            title={el.title}
            key={el.href}
          />
        ))}
      </div>
    </>
  );
};
