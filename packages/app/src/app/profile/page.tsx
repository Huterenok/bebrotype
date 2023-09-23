"use client";

import { Metadata } from "next";

import { ProfilePage } from "pages-flat";
import { useAuth } from "features/Auth";

export const metadata: Metadata = {
  title: "Profile",
  icons: "/icons/banana.svg",
};

export default function ProfileMePage() {
  const _ = useAuth();
  return <ProfilePage id={null}/>;
}
