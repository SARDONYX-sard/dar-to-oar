import dynamic from "next/dynamic";
import Loading from "@/components/pages/loading";

const Settings = dynamic(() => import("../../components/pages/settings"), {
  loading: () => <Loading />,
  ssr: false,
});

export default function Settings_() {
  return <Settings />;
}
