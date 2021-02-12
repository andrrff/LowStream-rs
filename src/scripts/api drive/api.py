from Google import Create_Service
import json

# filtragem dos dados 
def TypeFolder(animes):
    # declarações
    anime_ok = []
    # começo
    CLIENT_SECRENT_FILE = 'client_secret.json'
    API_NAME = 'drive'
    API_VERSION = 'v3'
    SCOPES = ['https://www.googleapis.com/auth/drive']
    service = Create_Service(CLIENT_SECRENT_FILE, API_NAME, API_VERSION, SCOPES)
    # faz tudo
    for anime_info in animes:
        # primeira vericação
        if anime_info['mimeType'] == "application/vnd.google-apps.folder":
            # request
            response_animes = service.files().list(pageSize=1000, supportsAllDrives=True, includeItemsFromAllDrives=True, q=f"'{anime_info['id']}' in parents",
                fields="files(id, name, mimeType), incompleteSearch, nextPageToken").execute()
            animes_p1 = response_animes.get('files', [])
            # loop do restado da request
            for anime_p1 in animes_p1:
                # segunda vericaçaõ
                if anime_p1['mimeType'] == "application/vnd.google-apps.folder":
                    # request
                    response_animes_p1 = service.files().list(pageSize=1000, supportsAllDrives=True, includeItemsFromAllDrives=True, q=f"'{anime_p1['id']}' in parents",
                        fields="files(id, name, mimeType), incompleteSearch, nextPageToken").execute()
                    animes_p2 = response_animes_p1.get('files', [])
                    print("função recursiva ativa")
                    # pra fazer vericação "infinita" 
                    # função recursiva
                    anime_ok = TypeFolder(animes_p2)
                # caso a segunda verificação for false
                else:
                    print("pasta 2")
                    print(f"Nome: {anime_p1['name']} id: {anime_p1['id']} MimeType: {anime_p1['mimeType']}")
                    anime_ok.append({ 'id': anime_p1['id'], 'name': anime_p1['name'], 'type': anime_p1['mimeType']  })
        # caso a primeira verificação for false
        else:
            print("pasta 1")
            print(f"Nome: {anime_info['name']} id: {anime_info['id']} MimeType: {anime_info['mimeType']}")
            anime_ok.append({ 'id': anime_info['id'], 'name': anime_info['name'], 'type': anime_info['mimeType']  })
    # tudo pronto
    return anime_ok
   

# ta porra
def Request_URSS():
    # declarações
    lista_animes = []
    # começo
    CLIENT_SECRENT_FILE = 'client_secret.json'
    API_NAME = 'drive'
    API_VERSION = 'v3'
    SCOPES = ['https://www.googleapis.com/auth/drive']
    service = Create_Service(CLIENT_SECRENT_FILE, API_NAME, API_VERSION, SCOPES)
    # id da pasta animes
    folder_id = '1iLoMmc8ycsRBGbuDn0-YMI-_H0m_HbFL'
    # faz a listagem completa
    # request
    response_pasta = service.files().list(pageSize=1000, supportsAllDrives=True, includeItemsFromAllDrives=True, q=f"'{folder_id}' in parents",
        fields="files(id, name, mimeType), incompleteSearch, nextPageToken").execute()
    # files é um json
    files = response_pasta.get('files', [])
    print("="*150)
    # nao a arquivos
    if not files:
        print('Pasta do anime nao encontrado .')
    else:
        print('Files: ')
        #lista todas as pasta da animes
        orderBy = 'name'
        for item in files:
            print("="*150)
            print(f"Nome: {item['name']} id: {item['id']} MimeType: {item['mimeType']}")
            print("_"*20)
            #lista os animes
            response_animes = service.files().list(pageSize=1000, supportsAllDrives=True, includeItemsFromAllDrives=True,  q=f"'{item['id']}' in parents",
                fields="files(id, name, mimeType), incompleteSearch, nextPageToken ").execute()
            animes = response_animes.get('files', [])
            # nao tem
            if not animes:
                print('Anime nao encontrado .')
            else:
                # função de filtragem dos dados
                anime_ok = TypeFolder(animes)              
            # add na lista principal
            lista_animes.append({'name':[item['name'], {'eps': anime_ok}]})
    # pronto
    return lista_animes


if __name__ == "__main__":
    # porra
    dados = Request_URSS()
    # balaiada
    print("T"*150)
    # salvo no json
    with open("Animes.json", "w") as json_file:
        # faz virar objeto do python
        json.dump(dados, json_file, indent=4)
