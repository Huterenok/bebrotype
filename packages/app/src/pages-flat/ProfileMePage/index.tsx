import { FC } from "react";

import { UserTexts } from "widgets/UserTexts";
import { Profile } from "enities/User";

import styles from "./ProfileMePage.module.scss";

interface ProfileMePageProps {
  id: number;
}

export const ProfileMePage: FC<ProfileMePageProps> = ({ id }) => {
  return (
    <section className={styles.wrapper}>
      <Profile id={id} />
      <UserTexts id={id} />
    </section>
  );
};
