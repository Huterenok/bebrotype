import { FC } from "react";
import Link from "next/link";

import { RunningBanana } from "widgets";

import styles from "./HomePage.module.scss";
import { Title, Text, Button } from "shared/ui";

export const HomePage: FC = () => {
  return (
    <div className={styles.wrapper}>
      <Title>Welcome to Bebrotype, bebrochad!</Title>
      <Text>
        Today was a hard day, right? It is difficult for all of us to overcome
        more and more new obstacles in order to reach heights. But I see that
        you are not one of those who are ready to avoid complexity and not
        self-improvement, so you ended up here - in Bebrotype
      </Text>
      <Text>
        Bebrotype is a place where you can upgrade your touch typing on your
        keyboard so that you can type like Ryan Gosling from Drive drives (you
        can say &apos;I drive&apos; all the time). Here you can type both simple
        texts and pieces of code in a variety of languages. Also here you can
        participate in various competitions and get nice prizes (only test NEAR
        are distributed now).
      </Text>
      <Button className={styles.btn}>
        <Link href={"/typing"}>Let&apos;s Start now!</Link>
      </Button>
      <div className={styles.model_wrapper}>
        <RunningBanana />
      </div>
    </div>
  );
};
