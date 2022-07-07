import Link from 'next/link';
import Image from 'next/image';

const links = [
  { name: 'Docs', href: '#href' },
  { name: 'Source', href: 'https://github.com/always-maap/Limoo-Lang' },
];

export default function Nav() {
  return (
    <div className="py-4">
      <div className="flex justify-between items-center">
        <Link href="/">
          <a>
            <Image src="/logo.png" width={48} height={48} alt="limoo-logo" />
          </a>
        </Link>
        <nav>
          <ul className="flex space-x-8">
            {links.map((link) => (
              <Link href={link.href} key={link.name}>
                <a>
                  <li className="text-lg font-medium">{link.name}</li>
                </a>
              </Link>
            ))}
          </ul>
        </nav>
      </div>
    </div>
  );
}
