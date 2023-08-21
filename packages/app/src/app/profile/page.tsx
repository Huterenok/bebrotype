import { Metadata } from "next";

import { ProfileInfo } from "widgets";

import styles from "app-flat/styles/page-styles/Profile.module.css";

export const metadata: Metadata = {
  title: "Profile",
  icons: "/icons/banana.svg",
};

export default function ProfilePage() {
  return (
    <div className={styles.wrapper}>
      <ProfileInfo />
    </div>
  );
}
