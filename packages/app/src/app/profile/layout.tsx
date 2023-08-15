"use client";

import { useAuth } from "enities/User/lib/useAuth";

export default function ProfileLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const isAuth = useAuth();

  return <section>{children}</section>;
}
