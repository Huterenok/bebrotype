"use client";

import { FC } from "react";

import { UserTexts } from "widgets/UserTexts";
import { useAuth } from "features/Auth";
import { Profile } from "enities/User";

import styles from "./ProfileMePage.module.scss";

export const ProfileMePage: FC = () => {
  const user = useAuth();

  return (
    <section className={styles.wrapper}>
      <Profile id={user.id} />
      <UserTexts id={user.id} />
    </section>
  );
};
