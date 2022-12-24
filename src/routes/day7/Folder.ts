import sum from '$lib/sum'
import File from './File'

class Folder {
	name: string
	parentFolder: Folder | null
	folders: Folder[] = []
	files: File[] = []

	constructor(name: string, parentFolder?: Folder) {
		this.name = name
		this.parentFolder = parentFolder ?? null
	}

	addContents = (input: string) => {
		const filesAndFolders = input.split('\n')

		filesAndFolders.forEach((fileOrFolderStr) => {
			const [size, name] = fileOrFolderStr.split(' ')

			if (size === 'dir') {
				this.folders.push(new Folder(name, this))
			} else {
				this.files.push(new File(name, Number(size)))
			}
		})
	}

	findChild = (name: string) => {
		if (name === '..') {
			if (!this.parentFolder) throw new Error(`ERROR: Cannot cd .. out of root folder!`)

			return this.parentFolder
		}

		const childFolder = this.folders.find((folder) => folder.name === name)

		if (!childFolder)
			throw new Error(`ERROR: Cannot find child folder with name ${name} in folder ${this.name}`)

		return childFolder
	}

	size = () => {
		const fileTotal = sum(this.files.map((file) => file.size))

		const folderTotal = sum(this.folders.map((folder) => folder.size()))

		return fileTotal + folderTotal
	}

	getAllChildFolders = () => {
		const allFolders: Folder[] = []

		const pushFolders = (folders: Folder[]) => {
			allFolders.push(...folders)

			folders.forEach((folder) => {
				if (folder.folders.length) {
					pushFolders(folder.folders)
				}
			})
		}

		pushFolders(this.folders)

		return allFolders
	}

	doesFolderContainDuplicateFiles = () => {
		const fileset = new Set()

		this.files.forEach((file) => {
			fileset.add(file)
		})

		return fileset.size !== this.files.length
	}
}

export default Folder
