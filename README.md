<p align="center">
    <img src="https://i.imgur.com/H0D5PfH.png" alt="Logo" width="160" height="160">

  <h3 align="center">๐ <b>Menhera Album</b> ๐</h3>

  <p align="center">
    An HTTP server made in Rust ๐ฆ to serve images to MenheraBot.
    <br />
    <a href="https://github.com/MenheraBot/MenheraBot"><strong>MenheraBot ยป</strong></a>
    <br />
    <br />
  </p>
</p>

## ๐จโ๐ป | Contributing

You may contribute to this project by creating a Pull Request to the `main` branch. You can also create a pull request just to add โจ **New Assets** โจ. Please follow the sequence of the images, naming the file as an incrementing ID.

## ๐ฅ | Running

To run Menhera Album, you need to have [Docker](https://www.docker.com/) in your machine. You have two options of installation, follow the one that applies to you.

### ๐ฎ | Building the Image

> If you want to build the image yourself, you can do it by following these steps:

1. ๐งน Clone the repository

```bash
git clone https://github.com/MenheraBot/MenheraAlbum.git
```

2. ๐ป Building the Image

```bash
docker build . --tag album
```

3. ๐โโ๏ธ Running a Container

```bash
docker run --name AlbumServer -p 8080:8080 -d -t album
```

That's It! You have a HTTP server at port 8080.

### ๐ | Downloading the Image

> If you don't really want all the source code, and just want to execute the bot, you can just donwload the image from the Container Registry.

1. ๐ฅ Download the image

```bash
docker pull ghcr.io/menherabot/album:latest
```

> You need to be [logged in](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-to-the-container-registry)

2. ๐โโ๏ธ Running a Container

```bash
docker run --name AlbumServer -p 8080:8080 -d -t ghcr.io/menherabot/album:latest
```

Vamoooooo!!! You have a HTTP server at port 8080.

## ๐จ | Made With

- [Actix](https://actix.rs/)

## โ๏ธ | License

Distributed under the MIT License. See `LICENSE` for more information.

## ๐ง | Contact

Discord: **Luxanna#5757**

Twitter: **[@Luxanna_Dev](https://twitter.com/Luxanna_Dev)**

---

MenheraBot was made with โค๏ธ by Luxanna.
