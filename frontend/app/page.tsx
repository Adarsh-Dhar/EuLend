import Image from "next/image";

export default function Home() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="space-y-4 p-6 bg-gray-100 rounded-md shadow-lg">
        <h1 className="">Welcome to the frontend</h1>
        <Image src="/images/logo.svg" alt="logo" width={200} height={200} />
      </div>
    </div>
  );
}
