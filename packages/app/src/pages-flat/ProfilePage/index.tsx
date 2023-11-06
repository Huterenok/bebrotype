import { FC } from "react";

import { UserTexts } from "widgets/UserTexts";
import { Profile } from "enities/User";

import styles from "./ProfilePage.module.scss";

interface ProfilePageProps {
  id: number;
}

export const ProfilePage: FC<ProfilePageProps> = ({ id }) => {
  return (
    <section className={styles.wrapper}>
      <Profile id={id} />
      <UserTexts id={id} />
    </section>
  );
};
